#ifndef example_h
#define example_h

/* This file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define COINJOIN_ENTRY_MAX_SIZE 9

#define MIN_COINJOIN_SESSIONS 1

#define MIN_COINJOIN_ROUNDS 2

#define MIN_COINJOIN_AMOUNT 2

#define MIN_COINJOIN_DENOMS_GOAL 10

#define MIN_COINJOIN_DENOMS_HARDCAP 10

#define MAX_COINJOIN_SESSIONS 10

#define MAX_COINJOIN_ROUNDS 16

#define MAX_COINJOIN_DENOMS_GOAL 100000

#define MAX_COINJOIN_DENOMS_HARDCAP 100000

#define MAX_COINJOIN_AMOUNT (MAX_MONEY / DUFFS)

#define DEFAULT_COINJOIN_SESSIONS 4

#define DEFAULT_COINJOIN_ROUNDS 4

#define DEFAULT_COINJOIN_AMOUNT 1000

#define DEFAULT_COINJOIN_DENOMS_GOAL 50

#define DEFAULT_COINJOIN_DENOMS_HARDCAP 300

#define COINJOIN_DENOM_OUTPUTS_THRESHOLD 500

#define COINJOIN_KEYS_THRESHOLD_WARNING 100

#define COINJOIN_KEYS_THRESHOLD_STOP 50

#define COINJOIN_RANDOM_ROUNDS 3

#define REFERENCE_DEFAULT_MIN_TX_FEE 1000

#define COINJOIN_QUEUE_TIMEOUT 30

#endif /* example_h */
