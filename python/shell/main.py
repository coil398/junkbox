import subprocess
from subprocess import PIPE

proc = subprocess.run("curl inet-ip.info", shell=True, stdout=PIPE, stderr=PIPE, text=True)
addr = proc.stdout
print(addr)
