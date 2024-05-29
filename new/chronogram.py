import socket
import time
import matplotlib.pyplot as plt

# Function to convert data to binary string
def data_to_binary(data):
    """Converts received data to a binary string representation."""
    return ''.join(f'{b:08b}' for b in data)

# Function to plot data in real-time as a chronogram
def plot_chronogram(conn):
    plt.ion()  # Enable interactive mode
    fig, ax = plt.subplots()
    ax.set_title("Vizualizing CANFD DATA")
    ax.set_xlabel('Time (s)')
    ax.set_ylabel('BCAN FD Data')
    ax.set_ylim(-0.5, 1.5)

    timestamps = []
    data_bits = []
    
    current_time = time.time()
    base_time = current_time
    bit_count = 0
    
    while True:
        try:
            data = conn.recv(1024)
            if not data:
                break
            binary_data = data_to_binary(data)[-100:]  # Pick the last 100 bits
            for bit in binary_data:
                timestamps.append(current_time - base_time)
                data_bits.append(int(bit))
                current_time += 1  # Increment current time by 1 second for each bit
                bit_count += 1
                
                # Plot data
                ax.step(timestamps, data_bits, where='post', color='blue')
                
                # Add vertical grid line for each bit
                ax.axvline(x=current_time - base_time, color='gray', linestyle='--', linewidth=0.5)
                
                ax.set_xlim(0, max(timestamps) + 1)
                plt.pause(0.001)  # Pause for a short duration to update the plot
                
                if bit_count >= 100:
                    break
        except Exception as e:
            print(f"Error plotting data: {e}")
            break
    
    # Ensure the final plot is updated
    plt.ioff()
    plt.show()

if __name__ == "__main__":
    # Start listening on port 5550
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.bind(('0.0.0.0', 5550))
        sock.listen()
        print("Listening for data on port 5550")

        # Accept incoming connections and handle them
        while True:
            conn, addr = sock.accept()
            print(f"Connected by {addr}")
            plot_chronogram(conn)
