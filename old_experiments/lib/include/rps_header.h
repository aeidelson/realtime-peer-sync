
#ifndef cheddar_generated_rps_header_h
#define cheddar_generated_rps_header_h


#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>



/** Common API **/
typedef struct RpsWorldState {
} RpsWorldState;

typedef struct RpsEvent {
} RpsEvent;

typedef RpsEvent RpsUserEvent;

typedef RpsEvent RpsCalculationEvent;

/** Client API **/
typedef struct RpsClientConfig {
	RpsCalculationEvent (*calculation_updater)(RpsWorldState world_state, uint64_t timeSinceLastUpdateMs);
	int32_t desired_calculation_frequency_hz;
} RpsClientConfig;

typedef struct RpsClient RpsClient;

RpsClient const* rps_new_client(void);

void dbg(RpsClient const* c_client);

/** Server API **/
typedef struct RpsServerConfig {
	RpsCalculationEvent (*calculate_updater)(RpsWorldState world_state, uint64_t timeSinceLastUpdateMs);
	int32_t desired_calculation_frequency_hz;
} RpsServerConfig;



#ifdef __cplusplus
}
#endif


#endif
