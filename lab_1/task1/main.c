#include "stdio.h"
#include "unistd.h"

int main() {
    // p1
    int pid = fork();
    if (pid == 0) {
        // p2
        pid = fork();
        if (pid == 0) {
            // p3
            sleep(20);
            return 0;
        }
        sleep(20);
        return 0;
    }
    pid = fork();
    if (pid == 0) {
        // p4
        pid = fork();
        if (pid == 0) {
            // p5
            sleep(20);
            return 0;
        }
        pid = fork();
        if (pid == 0) {
            // p6
            sleep(20);
            return 0;
        }
        sleep(20);
        return 0;
    }
    pid = fork();
    if (pid == 0) {
        // p7
        sleep(20);
        return 0;
    }

    sleep(20);
    return 0;
}