# rust-algorithm
Rust 实现链表等数据结构

## 技术取舍&思考过程

基础架构
1. 不使用 `Rc` 或 `unsafe`，而是使用 Arena 技术，类似 对象池

ArenaList 的管理方法
1. `ArenaList.nodes: Vec<Option<T>>` 用来存放数据对象
2. `nexts: Vec<Option<usize>>` 用来存放下一个节点的索引。若为 None，表示没有下一个节点。
3. 维护一个 `holes: Vec<usize>` 用来存放运行过程中产生的孔洞。代码里的 `holes` 用 **stack** 的方式还使用，也可以用 **queue** 的方式来使用（需要换成别的数据结构以保证性能）
4. 统一使用带 dummy 的节点，可以降低代码复杂性。
5. `LinkedList.root: unsize` 代表链表 dummy 节点位置。已有节点上生成另一个链表，只需要 new 一个新的 LinkedList。


ArenaList 数据类型的取舍
- 可以是 `nodes: Vec<Option<T>>` + `nexts: Vec<Option<usize>>`
- 也可以是 `nodes: Vec<NodeInfo<T>>`，其中 `NodeInfo{data, next_idx}`
- 应该没有优劣，我实现的是前者


ArenaList 要唯一
- 如果每个链表对应自己的 ArenaList，那么链表的 "拆分"、"合并" 这类原本 O(1) 的操作，会有大量 nodes 在不同 Vec 上批量移动。复杂度变成 O(n)
- 所以多个链表共用同一个 ArenaList. Tree/Graph 这类允许合并/拆分的数据结构也是一样。


维护 holes 的缺点和优点
- 优点：可以最大化利用内存，及时记录和使用孔洞。无须 `compact`
- 缺点：制造孔洞（del）、变更孔洞（make_node），都需要改变其上游节点对孔洞指向。例如删除需要把上游的 next 置为 None。这要求操作时知道父节点。
  - 这对于链表/二叉树是可能的，但是对于多叉树/图是不可能的

因此，对 `Graph` 需要增加一个 `compact` 方法来清理孔洞。
- 算法流程如下
  1. 对 holes 降序排序
  2. 把 `nodes.pop()` 放入 `nodes[holes.pop()]`。也就是最后一个有效节点放入第一个孔洞中
  3. 调整 nexts，把原本指向孔洞的值改为 None。把原本指向被移动的节点的，改为移动后的位置。
  4. 回到2，知道 holes 为空
- 整个 compact 的复杂度为 O(mn). m 为空洞数量，n 为 nodes 长度。相比之下，使用 holes 实时维护空洞复杂度为 O(1)。
- 别的地方需要做相应调整:
  - 删除只需要置 None, 判断为空的语句也相应调整。
  - 插入新节点时，不再寻找孔洞，而是插入到末尾。


使用 `holes` 维护空洞的优缺点
- 优点：添加节点和边的效率很高。删除节点的效率也不低（需要维护 holes）。
- 缺点：
    - `compact()` 时，遇到尾部是节点的情况，需要删除 `hole` 的第一个元素，但 `holes: Vec<usize>` 对这样的操作复杂度 为 `O(n)`
    - 每处理一个空洞，都需要遍历所有节点，因此复杂度为 O(mn)，其中 m 是空洞个数，n 是节点个数
- 改进：引入一个 `prev: Vec<usize>` 来记录节点的上游。
    - 缺点：额外维护关于边的数据量翻倍。新增边的性能损耗也高多一点（因为要更新 `prev`）。
    - 优点：`compact()` 处理单个空洞效率是 O(1)，总的效率为 O(m)。如果不维护 `holes` 总的效率为 O(n)
