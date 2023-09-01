#include <arpa/inet.h>
#include <linux/if_packet.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <net/if.h>
#include <netinet/ether.h>

#define BUF_SIZ         1024


int main(int argc, char *argv[])
{       char* intf = argv[1];
        int dest_mac_0 = (int)strtol(argv[2], NULL, 16);
        int dest_mac_1 = (int)strtol(argv[3], NULL, 16);
        int dest_mac_2 = (int)strtol(argv[4], NULL, 16);
        int dest_mac_3 = (int)strtol(argv[5], NULL, 16);
        int dest_mac_4 = (int)strtol(argv[6], NULL, 16);
        int dest_mac_5 = (int)strtol(argv[7], NULL, 16);
        int sockfd;
        struct ifreq if_idx;
        struct ifreq if_mac;
        int tx_len = 0;
        char sendbuf[BUF_SIZ];
        struct ether_header *eh = (struct ether_header *) sendbuf;
        struct iphdr *iph = (struct iphdr *) (sendbuf + sizeof(struct ether_header));
        struct sockaddr_ll socket_address;
        char ifName[IFNAMSIZ];

        /* Get interface name */
        if (argc > 1)
                strcpy(ifName, argv[1]);
        else
                strcpy(ifName, intf);

        /* Open RAW socket to send on */
        if ((sockfd = socket(AF_PACKET, SOCK_RAW, IPPROTO_RAW)) == -1) {
            perror("socket");
        }

        /* Get the index of the interface to send on */
        memset(&if_idx, 0, sizeof(struct ifreq));
        strncpy(if_idx.ifr_name, ifName, IFNAMSIZ-1);
        if (ioctl(sockfd, SIOCGIFINDEX, &if_idx) < 0)
            perror("SIOCGIFINDEX");
        /* Get the MAC address of the interface to send on */
        memset(&if_mac, 0, sizeof(struct ifreq));
        strncpy(if_mac.ifr_name, ifName, IFNAMSIZ-1);
        if (ioctl(sockfd, SIOCGIFHWADDR, &if_mac) < 0)
            perror("SIOCGIFHWADDR");

        /* Construct the Ethernet header */
        memset(sendbuf, 0, BUF_SIZ);
        /* Ethernet header */
        eh->ether_shost[0] = ((uint8_t *)&if_mac.ifr_hwaddr.sa_data)[0];
        eh->ether_shost[1] = ((uint8_t *)&if_mac.ifr_hwaddr.sa_data)[1];
        eh->ether_shost[2] = ((uint8_t *)&if_mac.ifr_hwaddr.sa_data)[2];
        eh->ether_shost[3] = ((uint8_t *)&if_mac.ifr_hwaddr.sa_data)[3];
        eh->ether_shost[4] = ((uint8_t *)&if_mac.ifr_hwaddr.sa_data)[4];
        eh->ether_shost[5] = ((uint8_t *)&if_mac.ifr_hwaddr.sa_data)[5];
        eh->ether_dhost[0] = dest_mac_0;
        eh->ether_dhost[1] = dest_mac_1;
        eh->ether_dhost[2] = dest_mac_2;
        eh->ether_dhost[3] = dest_mac_3;
        eh->ether_dhost[4] = dest_mac_4;
        eh->ether_dhost[5] = dest_mac_5;
        /* Ethertype field */
        eh->ether_type = htons(ETH_P_IP);
        tx_len += sizeof(struct ether_header);

        /* Packet data */
        sendbuf[tx_len++] = 0xde;
        sendbuf[tx_len++] = 0xad;
        sendbuf[tx_len++] = 0xbe;
        sendbuf[tx_len++] = 0xef;

        /* Index of the network device */
        socket_address.sll_ifindex = if_idx.ifr_ifindex;
        /* Address length*/
        socket_address.sll_halen = ETH_ALEN;
        /* Destination MAC */
        socket_address.sll_addr[0] = dest_mac_0;
        socket_address.sll_addr[1] = dest_mac_1;
        socket_address.sll_addr[2] = dest_mac_2;
        socket_address.sll_addr[3] = dest_mac_3;
        socket_address.sll_addr[4] = dest_mac_4;
        socket_address.sll_addr[5] = dest_mac_5;

        /* Send packet */
        if (sendto(sockfd, sendbuf, tx_len, 0, (struct sockaddr*)&socket_address, sizeof(struct sockaddr_ll)) < 0)
            printf("Send failed\n");

        return 0;
}
