import asyncio
from pyreccaster import PyReccaster
from p4p.nt import NTScalar
from p4p.server.asyncio import SharedPV
from p4p.server import Server

async def main():
    pv = SharedPV(nt=NTScalar('d'), initial=0.0)

    @pv.put
    def handle(pv, op):
        pv.post(op.value())
        print(f"{op.value()}")
        op.done()

    with Server(providers=[{"DEV:P4P:VAL": pv}]):
        py_reccaster = await PyReccaster.setup()
        await py_reccaster.run()


if __name__ == "__main__":
    asyncio.run(main())
