Tài liệu: https://docs.cosmos.network/v0.52/learn
## ** Cosmos SDK và Các Thành Phần Module**  

Cosmos SDK là một **framework** giúp xây dựng blockchain tùy chỉnh. Nó dựa trên kiến trúc **modular**, nghĩa là các blockchain có thể chọn module nào cần dùng để tạo hệ thống phù hợp với nhu cầu của mình.  

---

# **I. Cấu Trúc Cơ Bản của Cosmos SDK**
Cosmos SDK gồm phần chính:  
 **Base Layer:** Các thành phần cốt lõi như Tendermint, ABCI.  
 **Modules:** Các tính năng tùy chọn như Staking, IBC, Governance, CosmWasm.  
 **Application:** Logic ứng dụng riêng của blockchain.  


---

# ** II. Các Module Quan Trọng trong Cosmos SDK**
  
| **Module** | **Chức năng** | **Bắt buộc?** |
|------------|--------------|--------------|
| **auth** | Quản lý tài khoản & phí giao dịch | ✅ Có |
| **bank** | Xử lý token & giao dịch giữa tài khoản | ✅ Có |
| **staking** | Cơ chế Staking và Validators | ✅ Có |
| **slashing** | Phạt Validator gian lận hoặc offline | ✅ Có |
| **gov** | Quản trị on-chain (biểu quyết, đề xuất) | ✅ Có |
| **distribution** | Phân phối phần thưởng staking | ✅ Có |
| **mint** | Tạo token mới theo lịch trình | ✅ Có |
| **genutil** | Khởi tạo blockchain | ✅ Có |
| **evidence** | Kiểm tra và xử lý bằng chứng gian lận | ✅ Có |
| **params** | Quản lý thông số blockchain | ✅ Có |
| **upgrade** | Cập nhật & nâng cấp chain mà không hard fork | ✅ Có |
| **capability** | Module quản lý quyền hạn | ✅ Có |
| **vesting** | Token vesting (khóa token theo thời gian) | ❌ Không |
| **feegrant** | Cho phép tài khoản trả phí thay người khác | ❌ Không |
| **ibc** | Liên kết giữa blockchain Cosmos với nhau | ❌ Không |
| **wasm** | Chạy Smart Contract (CosmWasm) | ❌ Không |
| **evm** | Hỗ trợ Smart Contract Ethereum (Evmos) | ❌ Không |

 **Mặc định Cosmos SDK không có Smart Contract, nhưng có thể thêm IBC, CosmWasm hoặc EVM nếu cần.**  

---

# ** III. Cách Xây Dựng Blockchain Cosmos Tùy Chỉnh**
Bạn có thể dùng Cosmos SDK để **tạo blockchain riêng** bằng cách chọn module phù hợp.  

| **Binary**  | **Dùng để làm gì?** | **Hỗ trợ IBC?** | **Hỗ trợ Smart Contract?** | **Hỗ trợ EVM?** |
|-------------|-------------------|----------------|----------------------|----------------|
| **`gaiad`** | Chạy full node trên Cosmos Hub | ✅ Có | ❌ Không | ❌ Không |
| **`wasmd`** | Blockchain Cosmos hỗ trợ CosmWasm | ✅ Có | ✅ Có (CosmWasm) | ❌ Không |
| **`evmosd`** | Blockchain Cosmos hỗ trợ EVM (Ethereum Smart Contract) | ✅ Có | ✅ Có (CosmWasm) | ✅ Có (EVM) |
| **`osmosisd`** | AMM DEX trên Cosmos (giống Uniswap) | ✅ Có | ✅ Có (CosmWasm) | ❌ Không |
| **`injectived`** | Blockchain Cosmos hỗ trợ Smart Contract & DeFi | ✅ Có | ✅ Có (CosmWasm) | ✅ Có (EVM) |
| **`sei`** | Blockchain Cosmos tối ưu cho trading | ✅ Có | ✅ Có (CosmWasm) | ❌ Không |
| **`simd`** | Test blockchain Cosmos cục bộ | ❌ Không | ❌ Không | ❌ Không |


