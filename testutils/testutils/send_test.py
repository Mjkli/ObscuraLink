from scapy.all import *




# File to test spamming a network interface with packets to test capturing from rust side

def main():
    pkt = Ether()/IP(dst="127.0.0.1")/TCP(dport=80)

    sendp(pkt, iface="lo", loop=1, inter=0.2)

    
    




if __name__ == "__main__":
    main()
