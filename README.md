# Foreplay
> chatGPT调教前戏

## 路线图
1.读取GitHub项目：将GitHub项目克隆到本地，或使用GitHub API从远程获取项目文件。

2.转换项目为文本形式：遍历项目目录结构，将代码和文档转换为纯文本格式。可以使用os.walk()函数（Python）或tree命令（Linux和macOS）实现。

3.准备数据集：整理从GitHub项目中提取的文本，以便将其作为训练数据投喂给ChatGPT。您需要将文本划分为合适的输入输出对，以便模型能够从这些数据中学习。

4.微调ChatGPT：使用langchain的Memory生成记忆链让chagpt进行预热学习。

5.输出。
