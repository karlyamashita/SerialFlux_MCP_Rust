import json, re, html, datetime
from pathlib import Path
import tkinter as tk
from tkinter import ttk, messagebox
ROOT = Path(__file__).resolve().parent
HISTORY = ROOT / 'VersionHistory.json'
def key(v):
    if not re.fullmatch(r'(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)',v): raise ValueError('Use major.minor.patch')
    return tuple(map(int,v.split('.')))
def save_history(entries):
    entries = sorted(entries,key=lambda e:key(e['version']),reverse=True)
    if not entries or len({e['version'] for e in entries}) != len(entries): raise ValueError('History must contain unique versions')
    version=entries[0]['version']
    toml=(ROOT/'Cargo.toml').read_text(encoding='utf-8')
    lock=(ROOT/'Cargo.lock').read_text(encoding='utf-8')
    toml,n=re.subn(r'(?m)^(version\s*=\s*")[^"]+("\s*)$',lambda m:m[1]+version+m[2],toml,count=1)
    lock,k=re.subn(r'(?m)(^name = "serialflux-mcp"\nversion = ")[^"]+("$)',lambda m:m[1]+version+m[2],lock,count=1)
    if n!=1 or k!=1: raise ValueError('Package version not found')
    md='# Version history\n\n'+ '\n\n'.join('## '+e['version']+' — '+e['title']+ (' ('+e['date']+')' if e['date'] else '')+'\n'+'\n'.join('- '+c for c in e['changes']) for e in entries)+'\n'
    page=(ROOT/'Documentation/index.html').read_text(encoding='utf-8')
    section=''.join('<h3>'+html.escape(e['version']+' — '+e['title'])+'</h3>'+('<p>'+html.escape(e['date'])+'</p>' if e['date'] else '')+'<ul>'+''.join('<li>'+html.escape(c)+'</li>' for c in e['changes'])+'</ul>' for e in entries)
    page,n=re.subn(r'<!-- VERSION_HISTORY_START -->.*?<!-- VERSION_HISTORY_END -->',lambda m:'<!-- VERSION_HISTORY_START -->'+section+'<!-- VERSION_HISTORY_END -->',page,flags=re.S)
    if n!=1: raise ValueError('HTML history markers missing')
    for path,data in [(HISTORY,json.dumps(entries,indent=2,ensure_ascii=False)+'\n'),(ROOT/'Cargo.toml',toml),(ROOT/'Cargo.lock',lock),(ROOT/'CHANGELOG.md',md),(ROOT/'Documentation/index.html',page)]:
        path.write_text(data,encoding='utf-8')
    return entries

def main():
    entries=json.loads(HISTORY.read_text(encoding='utf-8')); selected=None
    root=tk.Tk(); root.title('Serial Flux MCP — Version Manager'); root.geometry('850x520')
    frame=ttk.Frame(root,padding=12); frame.pack(fill='both',expand=True)
    tree=ttk.Treeview(frame,columns=('version','date','title'),show='headings',height=9)
    for c,w in [('version',100),('date',110),('title',520)]: tree.heading(c,text=c.title()); tree.column(c,width=w)
    tree.pack(fill='x'); fields={}
    for label in ['version','date','title']:
        row=ttk.Frame(frame); row.pack(fill='x',pady=3); ttk.Label(row,text=label.title(),width=12).pack(side='left'); fields[label]=ttk.Entry(row); fields[label].pack(side='left',fill='x',expand=True)
    ttk.Label(frame,text='Changes (one per line)').pack(anchor='w'); changes=tk.Text(frame,height=7); changes.pack(fill='both',expand=True)
    def fill(e):
        for f in fields: fields[f].delete(0,'end'); fields[f].insert(0,e[f])
        changes.delete('1.0','end'); changes.insert('1.0','\n'.join(e['changes']))
    def refresh():
        tree.delete(*tree.get_children())
        for e in entries: tree.insert('','end',iid=e['version'],values=(e['version'],e['date'],e['title']))
    def select(_=None):
        nonlocal selected
        if tree.selection(): selected=tree.selection()[0]; fill(next(e for e in entries if e['version']==selected))
    def new():
        nonlocal selected
        selected=None; tree.selection_remove(*tree.selection()); a,b,c=key(entries[0]['version']); fill(dict(version=f'{a}.{b}.{c+1}',date=datetime.date.today().isoformat(),title='',changes=[]))
    def commit():
        nonlocal entries,selected
        try:
            e={f:fields[f].get().strip() for f in fields}; key(e['version']); e['changes']=[s.strip() for s in changes.get('1.0','end').splitlines() if s.strip()]
            if e['date']: datetime.date.fromisoformat(e['date'])
            if not e['title'] or not e['changes']: raise ValueError('Title and changes required')
            entries=save_history([x for x in entries if x['version']!=selected]+[e]); selected=e['version']; refresh(); tree.selection_set(selected); messagebox.showinfo('Saved','Version files synchronized.')
        except Exception as ex: messagebox.showerror('Error',str(ex))
    def delete():
        nonlocal entries,selected
        if selected and len(entries)>1 and messagebox.askyesno('Delete',f'Delete {selected}? Current version may change.'):
            try: entries=save_history([e for e in entries if e['version']!=selected]); refresh(); new()
            except Exception as ex: messagebox.showerror('Error',str(ex))
    tree.bind('<<TreeviewSelect>>',select); row=ttk.Frame(frame); row.pack(fill='x',pady=8)
    for label,fn in [('New Release',new),('Save Entry',commit),('Delete Entry',delete)]: ttk.Button(row,text=label,command=fn).pack(side='left',padx=4)
    refresh(); tree.selection_set(entries[0]['version']); select(); root.mainloop()
if __name__=='__main__': main()
