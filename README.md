# POE Benchmarks & Weight, Customizied Chainspec Lesson 6

#### Name: 彭亚伦
#### Phase: 4th
#### Team: #2
#### SID: 022

## Remark
本次提交作业是为第一课程写好的PoE模块编写benchmark和weight, 以及自定义Chainspec

### 1. 关于benchmark 和Weight
对于PoE模块里面定义的3个extrinsic, 可以很容易看出其计算逻辑和IO操作基本一致, 因此这里仅需对create_claim这个Call进行
benchmarking, 并按照claim的长度来判定weight, 由此3个call可以共用一个基准测试结果

### 2. Chain Spec 文件
1. plain 格式
[plain格式](https://raw.githubusercontent.com/Arstman/learn-poe-adv/main/chain-spec-plain.json)
2. raw 格式
[raw 格式](https://raw.githubusercontent.com/Arstman/learn-poe-adv/main/chain-spec.json)

## 本次作业Screenshots
### Benchmarks & Weight
![benchmarks and weights](https://raw.githubusercontent.com/Arstman/imgstorage/main/pic/benchmark_weight.png)


### Chain Specs
![chain specs](https://raw.githubusercontent.com/Arstman/imgstorage/main/pic/testnet.png)



# POE Pallet Practice for Substrate Learnning Course Lesson 1


## Screenshots
### pass all test
![tests passed](https://raw.githubusercontent.com/Arstman/learn-poe-adv/main/screenshots/poe-adv-test.png)

### running node
![running node after compile](https://raw.githubusercontent.com/Arstman/learn-poe-adv/main/screenshots/poe-adv-runnig.png)

### the frontend with TooLong Error
![frontend](https://raw.githubusercontent.com/Arstman/learn-poe-adv/main/screenshots/poe-adv-to-long.png)
