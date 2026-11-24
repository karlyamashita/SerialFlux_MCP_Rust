import asyncio
from pathlib import Path
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

async def main():
    exe = Path(__file__).resolve().parent.parent / 'serialflux-mcp.exe'
    async with stdio_client(StdioServerParameters(command=str(exe), args=[])) as (read, write):
        async with ClientSession(read, write) as session:
            await session.initialize()
            for name in ('sf_server_info', 'sf_list_connections'):
                result = await session.call_tool(name, {})
                print(result)

if __name__ == '__main__':
    asyncio.run(main())
