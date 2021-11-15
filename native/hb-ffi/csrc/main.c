#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
typedef struct Ws Ws;
extern Ws* start_ws(char* url);
// extern bool is_alive(Ws* ws);
extern void stop_ws(Ws* ws);
int main() {
    Ws* ws;
    // bool alive;
    ws = start_ws("wss://api.huobi.pro/ws");
    // alive = is_alive(ws);
    // printf("%d", &alive);
    stop_ws(ws);
    return 0;
}
