from scapy.all import *
import time



# File to test spamming a network interface with packets to test capturing from rust side

def main():
    data = "abc".encode('utf-8')
    pkt = Ether()/IP(dst="127.0.0.1")/TCP(dport=80)/Raw(load=data)
    
    sendp(pkt, loop=1,inter=1);
    




if __name__ == "__main__":
    main()
