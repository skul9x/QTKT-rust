# prompt_generator.py
import json
import tkinter as tk
from tkinter import filedialog, ttk

class App:
    def __init__(self, root):
        self.root = root
        self.root.title("JSON Prompt Generator")

        self.prompts = []

        top = tk.Frame(root)
        top.pack(fill="x", padx=10, pady=5)

        self.btn_load = tk.Button(top, text="Import TXT", command=self.load_file)
        self.btn_load.pack(side="left")

        self.btn_copy = tk.Button(top, text="Copy Prompt", command=self.copy_prompt)
        self.btn_copy.pack(side="left", padx=5)

        middle = tk.Frame(root)
        middle.pack(fill="both", expand=True, padx=10, pady=5)

        self.listbox = tk.Listbox(middle, width=40)
        self.listbox.pack(side="left", fill="y")

        scrollbar = tk.Scrollbar(middle)
        scrollbar.pack(side="left", fill="y")

        self.listbox.config(yscrollcommand=scrollbar.set)
        scrollbar.config(command=self.listbox.yview)

        self.text = tk.Text(middle)
        self.text.pack(side="left", fill="both", expand=True)

        self.listbox.bind("<<ListboxSelect>>", self.show_prompt)

    def generate_prompt(self, name):
        data = {
            "task": "Tìm kiếm online trên mạng các văn bản Bộ Y tế",
            "input_parameters": {
                "Ten_Quy_Trinh": name
            },
            "description": f"Tìm xem quy trình kỹ thuật có tên \"{name}\" có xuất hiện trong văn bản nào của Bộ Y tế.",
            "search_rules": {
                "document_type": "Quyết định của Bộ Y tế",
                "exclude": ["Thông tư"],
                "search_scope": "Danh mục quy trình kỹ thuật trong các quyết định ban hành hoặc cập nhật danh mục kỹ thuật"
            },
            "required_output": {
                "format": "table",
                "columns": [
                    "Số thứ tự văn bản tìm thấy",
                    "Tên văn bản (tên Quyết định của Bộ Y tế)",
                    "Số thứ tự của danh mục kỹ thuật trong văn bản đó",
                    "Ghi chú (nếu có)"
                ]
            }
        }
        return json.dumps(data, ensure_ascii=False, indent=2)

    def load_file(self):
        path = filedialog.askopenfilename(filetypes=[("Text Files", "*.txt")])
        if not path:
            return
        with open(path, "r", encoding="utf-8") as f:
            lines = [i.strip() for i in f.readlines() if i.strip()]
        self.prompts = [self.generate_prompt(i) for i in lines]
        self.listbox.delete(0, tk.END)
        for i in lines:
            self.listbox.insert(tk.END, i)

    def show_prompt(self, event):
        sel = self.listbox.curselection()
        if not sel:
            return
        idx = sel[0]
        self.text.delete("1.0", tk.END)
        self.text.insert(tk.END, self.prompts[idx])

    def copy_prompt(self):
        sel = self.listbox.curselection()
        if not sel:
            return
        idx = sel[0]
        self.root.clipboard_clear()
        self.root.clipboard_append(self.prompts[idx])
        self.root.update()

root = tk.Tk()
app = App(root)
root.mainloop()