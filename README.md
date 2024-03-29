# yuque_sae

语雀导出以及格式化

## 功能

1. 导出语雀个人版所有知识库文档
2. 导出语雀空间版所有知识库文档
3. 格式化导出的文档，也可以选择不导出直接格式化

> 提示：格式化时， **代码块** 和 **行内代码** 会被屏蔽，任何操作都不会影响 **代码块** 和 **行内代码** 内的内容

格式化功能包括：

1. 下载文档中的图片并替换文档中的 `url` 为本地路径，也可以选择不替换
2. 将语雀换行符 `<br />` 转换为两个空格加两个回车
3. 删除所有的 HTML 标签

## 使用

下载 [release](https://github.com/NPCDW/yuque_sae/releases) 程序与 `config.example.yml` 文件，并将其重命名为 `config.yml` 放在同一文件夹

按照自己的需求修改 `config.yml` 配置，如果需要导出语雀空间数据，获取语雀空间的 `cookie` 和 `x-crsf-token` 值，详见下图

打开命令行，在命令行中执行

```
yuque_sae.exe
```
执行与执行完成如下图：

![image](https://user-images.githubusercontent.com/32638459/202380837-b73eb31d-f5d7-40a0-b67b-54589593cfa8.png)

![image](https://user-images.githubusercontent.com/32638459/202380897-98c008c2-cecf-4a69-be56-0e7b57fc6606.png)

获取语雀空间的 `cookie` 和 `x-crsf-token` 值

![image](https://user-images.githubusercontent.com/32638459/212895811-187979e7-be95-46df-a4a7-545e37935e4d.png)

## 吐槽

1. 语雀的 `api` 只在 2018 年出了一版，之后 `api` 的功能再也没增加过，小记、表格、画板、演示文档都不能用 `api` 导出
2. 现在的语雀只能进不能出，导出功能通用格式只能一篇一篇导出，批量导出只支持语雀的私有格式
3. 编辑器图片无法引用自己图床或其他网站的高清图片，只能上传到语雀的 cdn ，然后被狠狠地压缩质量
4. UI 随便改，小记已归档被藏的很深
5. 功能随便砍，空间小记这种轻量级的备忘，直接被替换成重量级的知识库，知识小组被砍
