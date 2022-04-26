#define _GNU_SOURCE

#include <stdio.h>
#include <dlfcn.h>
#include <stdint.h>
#include <time.h>

#define GET_SYMBOL(var, sym)           \
    if (!(var))                        \
    {                                  \
        var = dlsym(RTLD_NEXT, (sym)); \
    }

// JSON - jansson
typedef enum
{
    JSON_OBJECT,
    JSON_ARRAY,
    JSON_STRING,
    JSON_INTEGER,
    JSON_REAL,
    JSON_TRUE,
    JSON_FALSE,
    JSON_NULL
} json_type;

typedef struct json_t
{
    json_type type;
    volatile size_t refcount;
} json_t;

#define json_typeof(json) ((json)->type)
#define json_is_object(json) ((json) && json_typeof(json) == JSON_OBJECT)
#define json_is_array(json) ((json) && json_typeof(json) == JSON_ARRAY)
#define json_is_string(json) ((json) && json_typeof(json) == JSON_STRING)
#define json_is_integer(json) ((json) && json_typeof(json) == JSON_INTEGER)
#define json_is_real(json) ((json) && json_typeof(json) == JSON_REAL)
#define json_is_number(json) (json_is_integer(json) || json_is_real(json))
#define json_is_true(json) ((json) && json_typeof(json) == JSON_TRUE)
#define json_is_false(json) ((json) && json_typeof(json) == JSON_FALSE)
#define json_boolean_value json_is_true
#define json_is_boolean(json) (json_is_true(json) || json_is_false(json))
#define json_is_null(json) ((json) && json_typeof(json) == JSON_NULL)

typedef json_t *(*jansson_object_get_t)(json_t *json, const char *key);
jansson_object_get_t x_jansson_object_get = NULL;
typedef const char *(*jansson_string_value_t)(const json_t *json);
jansson_string_value_t x_jansson_string_value = NULL;
typedef int (*jansson_integer_value_t)(const json_t *json);
jansson_integer_value_t x_jansson_integer_value = NULL;

// Conn/Cmd
typedef int (*conn_process_command_request_t)(int cmd, json_t *json_arg);
conn_process_command_request_t x_conn_process_command_request = NULL;

// Player
typedef void (*HI_Player_Pause_t)(void);
HI_Player_Pause_t x_HI_Player_Pause = NULL;
typedef void (*HI_Player_Play_t)(void);
HI_Player_Play_t x_HI_Player_Play = NULL;
typedef void (*HI_Player_Stop_t)(void);
HI_Player_Stop_t x_HI_Player_Stop = NULL;
typedef int (*HI_Player_VolumeGet_t)(void);
HI_Player_VolumeGet_t x_HI_Player_VolumeGet = NULL;
typedef int (*HI_Player_VolumeSet_t)(uint8_t vol);
HI_Player_VolumeSet_t x_HI_Player_VolumeSet = NULL;
typedef struct
{
    int status;
    uint8_t volume;
    uint8_t muted;
} global_player_t;
global_player_t *x_global_player = NULL;

// Screen
typedef int (*HI_Screen_InfoPack_t)(const char *s);
HI_Screen_InfoPack_t x_HI_Screen_InfoPack = NULL;
typedef int (*HI_Screen_Show_t)(void);
HI_Screen_Show_t x_HI_Screen_Show = NULL;
typedef int (*HI_Screen_BlinkColon_t)(int a);
HI_Screen_BlinkColon_t x_HI_Screen_BlinkColon = NULL;
typedef int (*HI_Acl_ReturnToIdleTimeSet_t)(int a);
HI_Acl_ReturnToIdleTimeSet_t x_HI_Acl_ReturnToIdleTimeSet = NULL;

// Sensor
typedef int (*MI_ADC_ReadLightSensor_t)(double *value);
MI_ADC_ReadLightSensor_t x_MI_ADC_ReadLightSensor = NULL;
typedef int (*MI_PIR_GetStatus_t)(unsigned int *value);
MI_PIR_GetStatus_t x_MI_PIR_GetStatus = NULL;
typedef int (*MI_PIR_GetStatusByIndex_t)(unsigned int index, unsigned int *value);
MI_PIR_GetStatusByIndex_t x_MI_PIR_GetStatusByIndex = NULL;

int y_get_pir_status()
{
    unsigned int value = 0;
    x_MI_PIR_GetStatus(&value);
    return value;
}

int y_get_pir_status_by_index(unsigned int index)
{
    unsigned int value = 0;
    x_MI_PIR_GetStatusByIndex(index, &value);
    return value;
}

double y_read_light_sensor()
{
    double value = 0.0;
    x_MI_ADC_ReadLightSensor(&value);
    return value;
}

int _Z28conn_process_command_requestiP9jansson_t(int cmd, json_t *json_arg)
{
    printf("called conn_process_command_request(%d, ...)\n", cmd);

    GET_SYMBOL(x_conn_process_command_request, "_Z28conn_process_command_requestiP9jansson_t");

    GET_SYMBOL(x_HI_Player_Pause, "_Z15HI_Player_Pausev");
    GET_SYMBOL(x_HI_Player_Play, "_Z14HI_Player_Playv");
    GET_SYMBOL(x_HI_Player_Stop, "_Z14HI_Player_Stopv");
    GET_SYMBOL(x_HI_Player_VolumeGet, "_Z19HI_Player_VolumeGetv");
    GET_SYMBOL(x_HI_Player_VolumeSet, "_Z19HI_Player_VolumeSeth");
    GET_SYMBOL(x_global_player, "global_player");

    GET_SYMBOL(x_jansson_object_get, "jansson_object_get");
    GET_SYMBOL(x_jansson_string_value, "jansson_string_value");
    GET_SYMBOL(x_jansson_integer_value, "jansson_integer_value");

    GET_SYMBOL(x_HI_Screen_InfoPack, "HI_Screen_InfoPack");
    GET_SYMBOL(x_HI_Screen_Show, "HI_Screen_Show");
    GET_SYMBOL(x_HI_Screen_BlinkColon, "HI_Screen_BlinkColon");
    GET_SYMBOL(x_HI_Acl_ReturnToIdleTimeSet, "_Z26HI_Acl_ReturnToIdleTimeSeti");

    GET_SYMBOL(x_MI_ADC_ReadLightSensor, "MI_ADC_ReadLightSensor");
    GET_SYMBOL(x_MI_PIR_GetStatus, "MI_PIR_GetStatus");
    GET_SYMBOL(x_MI_PIR_GetStatusByIndex, "MI_PIR_GetStatusByIndex");

    switch (cmd)
    {
    case 700:
        x_HI_Player_Play();
        return 0;

    case 701:
        x_HI_Player_Pause();
        return 0;

    case 702:
        x_HI_Player_Stop();
        return 0;

    case 703:
        return x_HI_Player_VolumeGet();

    case 704:
        if (!json_is_integer(json_arg))
        {
            return 1;
        }
        x_global_player->volume = x_jansson_integer_value(json_arg);
        return x_HI_Player_VolumeSet(x_jansson_integer_value(json_arg));

    case 750:;
        json_t *text_obj = x_jansson_object_get(json_arg, "text");
        if (!json_is_string(text_obj))
        {
            return 1;
        }
        const char *text = x_jansson_string_value(text_obj);

        int delay = 5;
        json_t *delay_obj = x_jansson_object_get(json_arg, "delay");
        if (json_is_integer(delay_obj))
        {
            delay = x_jansson_integer_value(delay_obj);
        }

        x_HI_Screen_InfoPack(text);
        x_HI_Screen_Show();
        x_HI_Screen_BlinkColon(delay);
        x_HI_Acl_ReturnToIdleTimeSet(delay);
        return 0;

    case 800:
        return y_get_pir_status();

    case 801:
        if (!json_is_integer(json_arg))
        {
            return -1;
        }
        return y_get_pir_status_by_index(x_jansson_integer_value(json_arg));

    case 802:
        return y_read_light_sensor() * 1000;

    default:
        return x_conn_process_command_request(cmd, json_arg);
    }
}


typedef void (*HI_Clock_Save_t)(void);
HI_Clock_Save_t x_HI_Clock_Save = NULL;

void _Z13HI_Clock_Savev(void) {
    static time_t last_save_time = 0;
    GET_SYMBOL(x_HI_Clock_Save, "_Z13HI_Clock_Savev");

    time_t time_now = time(NULL);

    if (time_now - last_save_time > 3600) {
        printf("saving clock config\n");
        
        x_HI_Clock_Save();
        last_save_time = time_now;
    } else {
        printf("skipped saving clock config (%ld)\n", time_now - last_save_time);
    }
}