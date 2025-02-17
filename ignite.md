
### **Kiểm tra phiên bản Ignite**
```bash
ignite version
```


### **Cài đặt Ignite CLI**
```bash
go install github.com/ignite/cli@latest
```

### **Khởi tạo blockchain mới**
```bash
ignite scaffold chain <tên_blockchain>
```
Ví dụ:
```bash
ignite scaffold chain myblockchain
cd myblockchain
```

### **Chạy blockchain cục bộ**
```bash
ignite chain serve
```

### **Xây dựng binary blockchain**
```bash
ignite chain build
```

---

## **2. Lệnh Scaffold - Tạo Cấu Trúc Mới** 🚀  

### **Tạo một module mới**
```bash
ignite scaffold module <tên_module>
```
Ví dụ:
```bash
ignite scaffold module blog
```

### **Tạo một message (giao dịch mới)**
```bash
ignite scaffold message <tên_message> <field_1> <field_2> --module <tên_module>
```
Ví dụ:
```bash
ignite scaffold message createPost title content --module blog
```

### **Tạo một loại dữ liệu mới (type)**
```bash
ignite scaffold type <tên_type> <field_1> <field_2> --module <tên_module>
```
Ví dụ:
```bash
ignite scaffold type Post title content author --module blog
```

### **Tạo một command CLI tùy chỉnh**
```bash
ignite scaffold command <tên_command> --module <tên_module>
```
Ví dụ:
```bash
ignite scaffold command myCommand --module mymodule
```

### **Tạo một query mới**
```bash
ignite scaffold query <tên_query> <field_1> <field_2> --module <tên_module>
```
Ví dụ:
```bash
ignite scaffold query getPost id --module blog
```

---

## **3. Lệnh Quản Lý Blockchain** 🏗  

### **Chạy blockchain**
```bash
ignite chain serve
```

### **Cập nhật và restart blockchain**
```bash
ignite chain reset
ignite chain serve
```

### **Chạy node testnet**
```bash
ignite chain init
ignite chain start
```

### **Export state của blockchain**
```bash
ignite chain export
```

### **Kiểm tra trạng thái node**
```bash
ignite chain status
```

---

## **4. Lệnh Quản Lý Tài Khoản & Giao Dịch** 🔑  

### **Tạo ví mới**
```bash
ignite key add <tên_user>
```
Ví dụ:
```bash
ignite key add alice
```

### **Liệt kê ví đã tạo**
```bash
ignite key list
```

### **Xóa một key**
```bash
ignite key delete <tên_user>
```

### **Chuyển token**
```bash
ignite tx bank send <người_gửi> <người_nhận> <số_lượng_token>stake
```
Ví dụ:
```bash
ignite tx bank send alice bob 100stake
```

### **Gọi một giao dịch**
```bash
ignite tx <module> <message> <tham_số> --from <tên_user>
```
Ví dụ:
```bash
ignite tx blog createPost "Tiêu đề" "Nội dung" --from alice
```

---

## **5. Lệnh Liên Quan Đến Smart Contract (Wasm)**
 **Ignite hỗ trợ triển khai smart contract CosmWasm trên Cosmos SDK.**

### **Triển khai smart contract**
```bash
ignite wasm deploy <đường_dẫn_tới_contract.wasm>
```

### **Gọi smart contract**
```bash
ignite wasm execute <địa_chỉ_contract> <tham_số> --from <tên_user>
```

### **Truy vấn smart contract**
```bash
ignite wasm query <địa_chỉ_contract> <tham_số>
```

---

## **6. Lệnh Dành Cho Quản Trị Blockchain (Governance)**
### **Tạo một proposal mới**
```bash
ignite tx gov submit-proposal <loại_proposal> <tham_số> --from <tên_user>
```
Ví dụ:
```bash
ignite tx gov submit-proposal text "Đề xuất mới" "Nội dung" --from alice
```

### **Vote cho một proposal**
```bash
ignite tx gov vote <id_proposal> <yes/no> --from <tên_user>
```
Ví dụ:
```bash
ignite tx gov vote 1 yes --from alice
```

---

## **7. Lệnh Quản Lý Mạng & Kết Nối** 🔄  

### **Chạy testnet với nhiều node**
```bash
ignite network chain create <tên_blockchain>
```

### **Thêm một node vào testnet**
```bash
ignite network chain join <tên_chain>
```

### **Danh sách các node trong mạng**
```bash
ignite network chain list
```

### **Đồng bộ dữ liệu giữa các node**
```bash
ignite network sync
```

---

## **8. Lệnh Debug & Kiểm Tra** 🛠  

### **Xem logs của node**
```bash
ignite chain logs
```

### **Xem thông tin block mới nhất**
```bash
ignite chain block latest
```

### **Xem danh sách validator**
```bash
ignite query staking validators
```

### **Kiểm tra trạng thái mạng**
```bash
ignite chain status
```

### ** change file genesis: add stake for admin
```bash
appd add-genesis-account $(ignite keys show admin -a) 1000000000stake
```