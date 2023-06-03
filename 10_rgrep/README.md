Grep Rsut 版实现

支持以下场景

1. 找到目标内容，打印包含内容的行，打印行数

   ```shell
   $ rgrep Hello a.txt
   55: Hello world. This is an exmaple text
   ```

2. 允许用户输入正则表达式

   ```shell
   $ rgrep Hel[^\\s]+ a.txt
   55: Hello world. This is an exmaple text
   89: Help me! I need assistant!
   ```

3. 允许用户提供一个正则表达式，来查找满足文件通配符的所有文件（你可以使用 globset 或者 glob 来处理通配符），比如

   ```shell
   $ rgrep Hel[^\\s]+ a*.txt
   a.txt 
   55:1 Hello world. This is an exmaple text 
   89:1 Help me! I need assistant! 
   5:6 Use `Help` to get help.
   abc.txt: 
   100:1 Hello Tyr!
   ```



提示

1. 对于命令行的部分，你可以使用 clap3 或者 structopt，也可以就用 env.args()。
2. 对于正则表达式的支持，可以使用 regex。
3. 至于文件的读取，可以使用 std::fs 或者 tokio::fs。你可以顺序对所有满足通配符的文件进行处理，也可以用 rayon 或者 tokio 来并行处理。
4. 对于输出的结果，最好能把匹配的文字用不同颜色展示