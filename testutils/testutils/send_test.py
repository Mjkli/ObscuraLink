from scapy.all import *
import time



# File to test spamming a network interface with packets to test capturing from rust side

def main():
    data = b"abc"
    pkt = Ether()/IP(dst="127.0.0.1")/UDP(sport=80, dport=80)/Raw(load=data)
    sendp(pkt, loop=1,inter=1, iface="lo");
    




if __name__ == "__main__":
    main()
