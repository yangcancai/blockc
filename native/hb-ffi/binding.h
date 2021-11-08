#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus
typedef struct Ws Ws;
int32_t error_message_utf8(char *buf, int32_t length);

bool is_alive(Ws *ws);

int32_t last_error_length(void);

int32_t load_page(int64_t port, const char *url);

int32_t start_timer(int64_t port);

Ws *start_ws(const char *url);

void stop_ws(Ws *ws);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
