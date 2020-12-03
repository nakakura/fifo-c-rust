#include <cstdio>
#include <cstdlib>
#include <string>
#include <iostream>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

const size_t BUFFER_SIZE(80);

int main()
{
    // ファイルディスクリプタ
    int fd;

    // FIFOの作成
    // mkfifo(<pathname>, <permission>)
    mkfifo("/tmp/myfifo", 0666);

    std::string message("Hello World!");
    // 書き込み専用でFIFOを開く
    fd = open("/tmp/myfifo", O_WRONLY);

    // メッセージの書き込み
    write(fd, message.c_str(), message.size() + 1);
    close(fd);

    return EXIT_SUCCESS;
}