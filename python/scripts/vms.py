import socket


def main():
    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    ip = socket.gethostbyname("127.0.0.1")
    port = 8484
    address = (ip, port)
    client.connect(address)
    while True:
        print("test1")
        data = client.recv(1024)
        print("test2")
        print(data)


if __name__ == "__main__":
    main()
