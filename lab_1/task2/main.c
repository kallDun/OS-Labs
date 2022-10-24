#include "sys/wait.h"
#include "stdio.h"
#include "unistd.h"

int main(int argc, char *argv[]) {
    int pid = fork();
    if (pid == 0) {

        char *arr[argc];
        for (int i = 0; i < argc - 1; ++i) {
            arr[i] = argv[i + 1];
        }
        arr[argc - 1] = NULL;

        execvp(arr[0], arr);
        return 0;

    } else {
        int status;
        waitpid(pid, &status, 0);

        if (status == 0){
            printf("Success!\n");
        }
        else{
            printf("Failed, exit code = %d\n", status);
        }
    }
    return 0;
}
