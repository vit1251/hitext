#!/usr/bin/env -S python3 -B -u

from argparse import ArgumentParser
from socket import socket, AF_INET, SOCK_DGRAM
from socket import SOL_SOCKET, SO_BROADCAST
from struct import pack
from time import sleep, time
from datetime import datetime
#from hexdump import hexdump
from pyfiglet import Figlet

class Color:
    Red = 1

class MsgType:
    Reset = 0
    Write = 1
    SetColor = 2
    GotoXY = 3

class Encoder:
    @staticmethod
    def make_reset_msg():
        msg = pack("!B", MsgType.Reset)
        return msg
    @staticmethod
    def make_write_msgs(chars: str):
        msgs = []
        for c in chars:
            c = c.encode()
            #print(c)
            msg = pack("!Bc", MsgType.Write, c)
            msgs.append(msg)
        return msgs
    @staticmethod
    def make_setcolor_msg(r: int, g: int, b: int):
        msg = pack("!BBBBB", MsgType.SetColor, r,g,b,0)
        return msg
    @staticmethod
    def make_gotoxy_msg(x: int, y: int):
        msg = pack("!BHH", MsgType.GotoXY, x, y)
        return msg

class MessageBuilder:
    def __init__(self):
        self.msgs = []
    def write(self, msg):
#        hexdump(msg)
        self.msgs.append(msg)
    def writes(self, msgs):
        for msg in msgs:
            self.write(msg)
    def build(self):
        return b''.join(self.msgs)

class Client:
    def __init__(self, addr, port=5000, broadcast=False):
        self.sock = socket(AF_INET, SOCK_DGRAM)
        if broadcast is True:
            self.sock.setsockopt(SOL_SOCKET, SO_BROADCAST, 1)
        self.addr = addr
        self.port = port

    def send(self, msg):
#        hexdump(msg)
        self.sock.sendto(msg, (self.addr, self.port))

def main():
    # Step 0. Parse arguments
    parser = ArgumentParser()
    parser.add_argument('addr', help="Network addres i.e. 192.168.1.1")
    parser.add_argument('--font', default='3-d', help="Font name")
    args = parser.parse_args()
    # Step 1. Create session
    c = Client(args.addr, broadcast=True)
    f = Figlet(font=args.font)
    while True:
        # Step 1. Make message
        now = datetime.now()
        stamp = now.strftime("%H:%M:%S")
        lines = f.renderText(stamp)
        # Step 2. Make message builder
        mb = MessageBuilder()
        mb.write(Encoder.make_reset_msg())
        mb.write(Encoder.make_setcolor_msg(128, 0, 0))
        for num, line in enumerate(lines.split('\n')):
            mb.write(Encoder.make_gotoxy_msg(18, 12 + num))
            mb.writes(Encoder.make_write_msgs(line))
        # Step 3. Make message packet and TX packet
        msgs = mb.build()
        c.send(msgs)
        # Step 4. Wait
        sleep(1)

if __name__ == "__main__":
    main()
