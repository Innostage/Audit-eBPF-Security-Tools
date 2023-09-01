#ifndef UDPCLIENT_H
#define UDPCLIENT_H

#include <iostream>
#include <cstring>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>

class UDPClient
{
public:
    UDPClient(unsigned short);
    void send(const char*);
private:
    unsigned short port;
    int sockfd;
    sockaddr_in serv_addr;
};

#endif
