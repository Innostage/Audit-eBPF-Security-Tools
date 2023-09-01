#include <stdio.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <strings.h>
#include <string.h>
#include <fcntl.h>
#include <stdlib.h>
#include "udpclient.h"

bool udpSend(const char *msg, char* dst_addr, int dst_port){
    sockaddr_in servaddr;
    int fd = socket(AF_INET,SOCK_DGRAM,0);
    if(fd<0){
        perror("cannot open socket");
        return false;
    }

    bzero(&servaddr,sizeof(servaddr));
    servaddr.sin_family = AF_INET;
    servaddr.sin_addr.s_addr = inet_addr(dst_addr);
    servaddr.sin_port = htons(dst_port);
    if (sendto(fd, msg, strlen(msg) + 1, 0, // +1 to include terminator
               (sockaddr*)&servaddr, sizeof(servaddr)) < 0){
        perror("cannot send message");
        close(fd);
        return false;
    }
    close(fd);
    return true;
}

int main(int argc, char *argv[]){
    const char* msg = "Jane Doe";
    char* dst_addr = argv[1];
    int dst_port = atoi(argv[2]);
    udpSend(msg, dst_addr, dst_port);
}
