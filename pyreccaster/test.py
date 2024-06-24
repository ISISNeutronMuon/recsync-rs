import asyncio
from pyreccaster import PyReccaster, PyRecord
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

    records = [
        PyRecord(name="DEV:P4P:TEST", type="ai", alias=None, properties={"recordDesc": "Test ai record"}),
        PyRecord(name="DEV:P4P:VAL", type="longin", alias=None, properties={"recordDesc": "Test longin record"}),
    ]

    with Server(providers=[{"DEV:P4P:VAL": pv}]):
        py_reccaster = await PyReccaster.setup(records)
        await py_reccaster.run()


if __name__ == "__main__":
    asyncio.run(main())
