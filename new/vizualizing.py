"""import socket

# Define the server address and port
HOST = '0.0.0.0'  # Listen on all available interfaces
PORT = 5550       # Port to listen on

# Create a TCP/IP socket
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# Bind the socket to the address and port
server_socket.bind((HOST, PORT))

# Listen for incoming connections (allow up to 5 queued connections)
server_socket.listen(5)

print(f"Server listening on {HOST}:{PORT}")

while True:
    # Wait for a connection
    client_socket, client_address = server_socket.accept()
    print(f"Connection from {client_address}")

    try:
        # Receive data in small chunks
        while True:
            data = client_socket.recv(1024)
            if data:
                # Print each byte as a decimal number
                for byte in data:
                    print(byte)
            else:
                # No more data from the client
                break
    finally:
        # Clean up the connection
        client_socket.close()
"""

import socket
import time
import requests

# Define the server address and port
HOST = '0.0.0.0'  # Listen on all available interfaces
PORT = 5550       # Port to listen on

# ThingSpeak API settings
THINGSPEAK_WRITE_API_KEY = '3U4JM5Z5OD72EF2T'
THINGSPEAK_URL = 'https://api.thingspeak.com/update?api_key=3U4JM5Z5OD72EF2T&field1=0'

# Create a TCP/IP socket
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# Bind the socket to the address and port
server_socket.bind((HOST, PORT))

# Listen for incoming connections (allow up to 5 queued connections)
server_socket.listen(5)

print(f"Server listening on {HOST}:{PORT}")

def send_to_thingspeak(value):
    payload = {'api_key': THINGSPEAK_WRITE_API_KEY, 'field1': value}
    response = requests.get(THINGSPEAK_URL, params=payload)
    if response.status_code == 200:
        print(f"Successfully sent data to ThingSpeak: {value}")
    else:
        print(f"Failed to send data to ThingSpeak. Status code: {response.status_code}")

while True:
    # Wait for a connection
    client_socket, client_address = server_socket.accept()
    print(f"Connection from {client_address}")

    try:
        # Receive data in small chunks
        while True:
            data = client_socket.recv(1024)
            if data:
                # Print each byte as a decimal number and send to ThingSpeak
                for byte in data:
                    print(byte)
                    send_to_thingspeak(byte)
                    time.sleep(0.5)  # Wait for 0.5 seconds before sending the next byte
            else:
                # No more data from the client
                break
    finally:
        # Clean up the connection
        client_socket.close()
