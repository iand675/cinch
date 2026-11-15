#include "HostApi.h"
int main(void) {
    if (cinch_export_HostApi_add(20, 22) != 42) return 1;
    void* counter = cinch_export_HostApi_Counter_new();
    if (cinch_export_HostApi_Counter_add(counter, 41) != 42) return 2;
    cinch_export_HostApi_Counter_delete(counter);
    return cinch_export_HostApi_destroyed() == 42 ? 0 : 3;
}
