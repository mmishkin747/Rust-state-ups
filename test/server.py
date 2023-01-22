import sys
import socket


server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.setblocking(False)
server.bind(('localhost', int(sys.argv[1])))
server.listen(5)

connections = []

while True:
    try:
        connection, address = server.accept()
        connection.setblocking(False)
        
        connections.append(connection)
    except BlockingIOError:
        pass

    for connection in connections:
        try:
            message = connection.recv(4096)
        except BlockingIOError:
            continue

        if message: 
        
            match message:
                case b'O\r\n' :
                    connection.send(b'221\r\n')
                case b'P\r\n':
                    connection.send(b'17\r\n')
                case b'C\r\n':
                    connection.send(b'21\r\n')
                case b'0\r\n':
                    connection.send(b'99\r\n')
                case b'j\r\n':
                    connection.send(b'35\r\n')
