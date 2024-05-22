import socket

def start_server():
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind(('0.0.0.0', 5700))
    server_socket.listen(1)
    print("Server listening on port 5700")

    while True:
        conn, addr = server_socket.accept()
        print(f"Connection from {addr}")
        try:
            while True:
                data = conn.recv(4096)
                if not data:
                    break
                print(data.decode('utf-8'), end='')
        except ConnectionResetError:
            print("Connection reset by peer")
        finally:
            conn.close()

if __name__ == "__main__":
    start_server()
