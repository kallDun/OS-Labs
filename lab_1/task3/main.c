#include <stdbool.h>
#include "sys/wait.h"
#include "stdio.h"
#include "unistd.h"
#include "string.h"

size_t symbols_len;
int cursor = 0;
char* symbols;
char symbol;

void ForkExec();

int main(int argc, char *argv[]) {
    symbols = argv[1];
    symbols_len = strlen(symbols);
    ForkExec();
    sleep(20);
    return 0;
}

void ForkExec() {
    int pid = -1;
    bool prevDot = false;
    int bracesIn = 0;

    while (symbols_len > cursor){
        symbol = symbols[cursor++];

        if (pid != 0 && bracesIn > 0){
            if (symbol == '['){
                bracesIn++;
            }
            if (symbol == ']'){
                bracesIn--;
            }
        }
        else if (symbol == '.'){
            if (prevDot && pid == 0){
                return;
            }
            pid = fork();
            prevDot = true;
        }
        else if (symbol == '[' && prevDot){
            if (pid == 0){
                ForkExec();
                return;
            }
            else{
                bracesIn = 1;
                prevDot = false;
            }
        }
        else if (symbol == ']') return;
    }
}