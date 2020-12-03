#include <cstdio>
#include <cstdlib>
#include <string>
#include <iostream>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

const size_t BUFFER_SIZE(1500);

int main()
{
    // ファイルディスクリプタ
    int fd;

    // FIFOの作成
    // mkfifo(<pathname>, <permission>)
    mkfifo("/tmp/myfifo", 0666);

    char str[BUFFER_SIZE];
    // 読み込み専用でFIFOを開く
    std::cout << "before open" << std::endl;
    fd = open("/tmp/myfifo", O_RDONLY);
    std::cout << "after open" << std::endl;
    while(1) {
        std::cout << "read start" << std::endl;
        ssize_t len = read(fd, str, BUFFER_SIZE);
        const std::string message(str);

        // 読み込んだ内容の表示
        std::cout << len << std::endl;
    }
    close(fd);

    return EXIT_SUCCESS;
}
