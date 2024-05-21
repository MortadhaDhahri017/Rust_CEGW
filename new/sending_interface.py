import tkinter as tk
from tkinter import ttk, messagebox
import socket
import struct

class ScrollableFrame(ttk.Frame):
    def __init__(self, container):
        super().__init__(container)
        canvas = tk.Canvas(self)
        scrollbar = ttk.Scrollbar(self, orient="vertical", command=canvas.yview)
        self.scrollable_frame = ttk.Frame(canvas)

        self.scrollable_frame.bind(
            "<Configure>",
            lambda e: canvas.configure(
                scrollregion=canvas.bbox("all")
            )
        )

        canvas.create_window((0, 0), window=self.scrollable_frame, anchor="nw")
        canvas.configure(yscrollcommand=scrollbar.set)

        canvas.pack(side="left", fill="both", expand=True)
        scrollbar.pack(side="right", fill="y")

class App(tk.Tk):
    def __init__(self):
        super().__init__()

        self.title("Structs Input Form")
        self.geometry("800x600")

        style = ttk.Style(self)
        style.theme_use("clam")
        style.configure("TLabel", font=("Arial", 12))
        style.configure("TButton", font=("Arial", 12))
        style.configure("TEntry", font=("Arial", 12))

        container = ScrollableFrame(self)
        container.pack(fill="both", expand=True, padx=10, pady=10)

        self.fields = {}
        self.create_ethframe(container.scrollable_frame)
        self.create_ipv4header(container.scrollable_frame)
        self.create_tcpheader(container.scrollable_frame)
        self.create_fsc(container.scrollable_frame)

        send_button = ttk.Button(self, text="Send Data", command=self.send_data)
        send_button.pack(pady=10)

    def create_ethframe(self, parent):
        self.create_section_label(parent, "EthFrame")
        self.create_array_fields(parent, "eth_dst", 6, ["00", "1A", "2B", "3C", "4D", "5E"])
        self.create_array_fields(parent, "eth_src", 6, ["5E", "4D", "3C", "2B", "1A", "00"])
        self.create_field(parent, "ethertype", 2, "0800")  # Assuming ethertype is u16

    def create_ipv4header(self, parent):
        self.create_section_label(parent, "Ipv4Header")
        self.create_field(parent, "version", 1, "4")
        self.create_field(parent, "len", 1, "5")
        self.create_field(parent, "ToS", 1, "00")
        self.create_field(parent, "total_len", 2, "003C")
        self.create_field(parent, "id", 2, "1C46")
        self.create_field(parent, "flags", 1, "2")
        self.create_field(parent, "frag_offset", 1, "00")
        self.create_field(parent, "ttl", 1, "40")
        self.create_field(parent, "protocol", 1, "06")
        self.create_field(parent, "checksum", 2, "B1E6")
        self.create_array_fields(parent, "ipv4_src", 4, ["C0", "A8", "00", "01"])
        self.create_array_fields(parent, "ipv4_dst", 4, ["C0", "A8", "00", "C7"])

    def create_tcpheader(self, parent):
        self.create_section_label(parent, "TcpHeader")
        self.create_field(parent, "src_port", 2, "04D2")
        self.create_field(parent, "dst_port", 2, "0050")
        self.create_field(parent, "seq", 4, "00000000")
        self.create_field(parent, "ack", 4, "00000000")
        self.create_field(parent, "offset", 1, "5")
        self.create_field(parent, "reserved", 1, "00")
        self.create_field(parent, "flags", 1, "02")
        self.create_field(parent, "window", 2, "7210")
        self.create_field(parent, "checksum", 2, "0000")
        self.create_field(parent, "urgent_ptr", 2, "0000")
        self.create_array_fields(parent, "tcp_data", 4, ["00", "00", "00", "00"])

    def create_fsc(self, parent):
        self.create_section_label(parent, "FSC")
        self.create_array_fields(parent, "fsc", 4, ["00", "00", "00", "00"])

    def create_section_label(self, parent, text):
        ttk.Label(parent, text=text, font=("Arial", 16, "bold")).pack(anchor="w", pady=(10, 5))

    def create_field(self, parent, name, byte_length, default_value="00"):
        frame = ttk.Frame(parent)
        frame.pack(anchor="w", pady=2, fill='x')
        ttk.Label(frame, text=f"{name}:").pack(side="left", padx=(0, 10))
        entry_list = []
        for i in range(byte_length):
            entry = ttk.Entry(frame, width=5)
            entry.pack(side="left", padx=2)
            entry.insert(0, default_value[i*2:(i+1)*2])  # Insert default value (2 chars per byte)
            entry_list.append(entry)
        self.fields[name] = entry_list

    def create_array_fields(self, parent, name, count, default_values):
        frame = ttk.Frame(parent)
        frame.pack(anchor="w", pady=2, fill='x')
        ttk.Label(frame, text=f"{name}:").pack(side="left", padx=(0, 10))
        entry_list = []
        for i in range(count):
            entry = ttk.Entry(frame, width=5)
            entry.pack(side="left", padx=2)
            entry.insert(0, default_values[i])
            entry_list.append(entry)
        self.fields[name] = entry_list

    def send_data(self):
        try:
            data = self.collect_data()
            if data is not None:
                with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
                    sock.connect(('127.0.0.1', 5400))
                    sock.sendall(data)
                messagebox.showinfo("Success", "Data sent successfully!")
        except Exception as e:
            messagebox.showerror("Error", f"Failed to send data: {e}")

    def collect_data(self):
        data = b''
        field_order = [
            'eth_dst', 'eth_src', 'ethertype',
            'version', 'len', 'ToS', 'total_len', 'id', 'flags', 'frag_offset', 'ttl', 'protocol', 'checksum',
            'ipv4_src', 'ipv4_dst',
            'src_port', 'dst_port', 'seq', 'ack', 'offset', 'reserved', 'flags', 'window', 'checksum', 'urgent_ptr', 'tcp_data',
            'fsc'
        ]
        try:
            for name in field_order:
                entries = self.fields[name]
                for entry in entries:
                    value = int(entry.get(), 16)  # Read as hexadecimal
                    if name in ["ethertype", "src_port", "dst_port", "total_len", "id", "checksum", "window", "urgent_ptr"]:
                        data += struct.pack('!H', value)
                    elif name in ["seq", "ack", "fsc"]:
                        data += struct.pack('!I', value)
                    else:
                        data += struct.pack('!B', value)
            return data
        except ValueError:
            messagebox.showerror("Input Error", "Please enter valid hexadecimal values in all fields.")
            return None

if __name__ == "__main__":
    app = App()
    app.mainloop()

