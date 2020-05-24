import asyncio
import time
import requests
import functools
from concurrent.futures import ThreadPoolExecutor

async def aSleep(seconds, value):
    await asyncio.sleep(seconds)
    return value * 10

def synchronousFunc(Object: dict, number):
    print('number: ', number)
    return number

async def aSleep2(loop, number):
    func = functools.partial(synchronousFunc, Object=dict(), number=number)
    res = await loop.run_in_executor(None, func)
    return res

loop = asyncio.get_event_loop()
print(loop.run_until_complete(asyncio.gather(aSleep(1, 10), aSleep(3, 30), aSleep(2, 20))))
loop = asyncio.get_event_loop()
print(loop.run_until_complete(asyncio.gather(aSleep2(loop, 1), aSleep2(loop, 2), aSleep2(loop, 3), aSleep2(loop, 4), aSleep2(loop, 5))))

def synchronousFunc2(num):
    print(num)
    return num

with ThreadPoolExecutor(2) as e:
    ret = e.map(synchronousFunc2, [number for number in range(1, 6)])

for res in ret:
    print('res: ', res)

def synchronousFuncWrapper(d):
    print('d: ', d)
    res = synchronousFunc(Object=d[0], number=d[1])
    return res

with ThreadPoolExecutor(2) as e:
    ret = e.map(synchronousFuncWrapper, [(dict(), number) for number in range(1, 6)])

for res in ret:
    print('res: ', res)
