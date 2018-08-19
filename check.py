# Script to chek if the examples build 
# As a side effect, it also checks if rgl builds

import os 
from subprocess import run

def check():
    run(['cargo', 'check'])

os.chdir("Examples/")

for dir in os.listdir("."):
    os.chdir(dir + "/")
    print ("\n" + "=" * 20)
    print ("Checking example: " + dir, end = "\n\n")
    check()
    os.chdir("..")