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

        send_button = ttk.Button(self, text="Send Data", command=self.send_data)
        send_button.pack(pady=10)

    def create_ethframe(self, parent):
        self.create_section_label(parent, "EthFrame")
        self.create_array_fields(parent, "dst", 6)
        self.create_array_fields(parent, "src", 6)
        self.create_field(parent, "ethertype", 2)  # Assuming ethertype is u16

    def create_ipv4header(self, parent):
        self.create_section_label(parent, "Ipv4Header")
        self.create_field(parent, "version", 1)
        self.create_field(parent, "len", 1)
        self.create_field(parent, "ToS", 1)
        self.create_field(parent, "total_len", 2)
        self.create_field(parent, "id", 2)
        self.create_field(parent, "flags", 1)
        self.create_field(parent, "frag_offset", 1)
        self.create_field(parent, "ttl", 1)
        self.create_field(parent, "protocol", 1)
        self.create_field(parent, "checksum", 2)
        self.create_array_fields(parent, "src", 4)
        self.create_array_fields(parent, "dst", 4)

    def create_tcpheader(self, parent):
        self.create_section_label(parent, "TcpHeader")
        self.create_field(parent, "src_port", 2)
        self.create_field(parent, "dst_port", 2)
        self.create_field(parent, "seq", 4)
        self.create_field(parent, "ack", 4)
        self.create_field(parent, "offset", 1)
        self.create_field(parent, "reserved", 1)
        self.create_field(parent, "flags", 1)
        self.create_field(parent, "window", 2)
        self.create_field(parent, "checksum", 2)
        self.create_field(parent, "urgent_ptr", 2)
        self.create_array_fields(parent, "data", 4)

    def create_section_label(self, parent, text):
        ttk.Label(parent, text=text, font=("Arial", 16, "bold")).pack(anchor="w", pady=(10, 5))

    def create_field(self, parent, name, byte_length):
        frame = ttk.Frame(parent)
        frame.pack(anchor="w", pady=2, fill='x')
        ttk.Label(frame, text=f"{name}:").pack(side="left", padx=(0, 10))
        entry_list = []
        for _ in range(byte_length):
            entry = ttk.Entry(frame, width=5)
            entry.pack(side="left", padx=2)
            entry_list.append(entry)
        self.fields[name] = entry_list

    def create_array_fields(self, parent, name, count):
        frame = ttk.Frame(parent)
        frame.pack(anchor="w", pady=2, fill='x')
        ttk.Label(frame, text=f"{name}:").pack(side="left", padx=(0, 10))
        entry_list = []
        for _ in range(count):
            entry = ttk.Entry(frame, width=5)
            entry.pack(side="left", padx=2)
            entry_list.append(entry)
        self.fields[name] = entry_list

    def send_data(self):
        try:
            data = self.collect_data()
            if data is not None:
                with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
                    sock.connect(('10.0.2.15', 5400))
                    sock.sendall(data)
                messagebox.showinfo("Success", "Data sent successfully!")
        except Exception as e:
            messagebox.showerror("Error", f"Failed to send data: {e}")

    def collect_data(self):
        data = b''
        try:
            for name, entries in self.fields.items():
                for entry in entries:
                    value = int(entry.get())
                    if name == "ethertype" or name.endswith("port") or name.endswith("len") or name == "id" or name == "checksum" or name == "window" or name == "urgent_ptr":
                        data += struct.pack('!H', value)
                    elif name == "seq" or name == "ack":
                        data += struct.pack('!I', value)
                    else:
                        data += struct.pack('!B', value)
            return data
        except ValueError:
            messagebox.showerror("Input Error", "Please enter valid integers in all fields.")
            return None

if __name__ == "__main__":
    app = App()
    app.mainloop()
