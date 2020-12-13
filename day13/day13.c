#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <regex.h>
#include <stdint.h>
#include <stdbool.h>

#include "intlist.h"

#define INT_STR_BUF_SIZE 32

/** 
 * Parse all bus IDs from file
 * Include x as -1
 */
IntList_t parse_ids(const char* contents)
{
    IntList_t ids = ilist_alloc();

    const char* s  = contents;
    regex_t regex;
    regmatch_t pmatch[1];
    regoff_t off, len;

    char buf[INT_STR_BUF_SIZE];

    const char* id_pat = "[0-9x]+";

    if (regcomp(&regex, id_pat, REG_EXTENDED))
        exit(EXIT_FAILURE);

    while (regexec(&regex, s, 1, pmatch, 0) == 0)
    {
        off = pmatch[0].rm_so;
        len = pmatch[0].rm_eo - pmatch[0].rm_so;

        if (s[off] == 'x')
        {
            ilist_push(&ids, 0);
        }
        else
        {
            assert(len+1 < INT_STR_BUF_SIZE);
            memcpy(buf, s + off, len);
            buf[len] = 0;
            uint64_t id = atoi(buf);
            ilist_push(&ids, id);
        }

        s += pmatch[0].rm_eo;
    }

    return ids;
}

uint64_t part1_nextbus(IntList_t ids)
{
    uint64_t depart = ids.d[0];

    uint64_t minwait = 10000000;
    uint64_t minbusid = 0;

    for (size_t i = 1; i < ids.len; i++)
    {
        uint64_t busid = ids.d[i];

        if (busid > 0)
        {
            uint64_t m = depart % busid;
            uint64_t next = busid - m;

            if (next < minwait)
            {
                minwait = next;
                minbusid = busid;
            }
        }
    }

    return minwait*minbusid;
}

typedef struct 
{
    uint64_t id;
    uint64_t offs;
} Bus_t;

size_t create_buses(IntList_t ids, Bus_t** buses_out)
{
    size_t count = 0;
    for (size_t i = 1; i < ids.len; i++)
    {
        if (ids.d[i] > 0)
        {
            count++;
        }
    }

    Bus_t* buses = (Bus_t*)malloc(sizeof(Bus_t)*count);

    size_t bi = 0;
    for (size_t i = 1; i < ids.len; i++)
    {
        if (ids.d[i] > 0)
        {
            Bus_t bus;
            bus.id = ids.d[i];
            bus.offs = ids.len - i - 1;
            buses[bi++] = bus;
        }
    }

    *buses_out = buses;
    return count;
}

Bus_t collide(Bus_t a, Bus_t b)
{
    // Find first solution to "collision"
    // If each bus is represented by nx+c
    // You need n1*x1+c1 = n2*x2+c2
    // Find first value of n1 such that
    // (n1*x1 + c1) % x2 == c2 % x2
    // Then find the value at this point, the new offset:
    //  n1x1 + c1 (== n2x2+c2)
    // And then the period(id) = LCM(x1, x2) = x1*x2 (because prime)

    uint64_t n1;
    uint64_t target = b.offs % b.id;
    for (n1 = 1; (n1 * a.id + a.offs) % b.id != target; n1++)
    { }

    Bus_t newbus;
    newbus.offs = n1*a.id + a.offs;
    newbus.id = a.id*b.id;
    return newbus;
}

uint64_t part2_synctime(IntList_t ids)
{
    Bus_t* buses;
    size_t count = create_buses(ids, &buses);

    Bus_t acc = buses[0];
    for (size_t i = 1; i < count; i++)
    {
        acc = collide(acc, buses[i]);
    }

    free(buses);

    return acc.offs - (ids.len - 2);
}

void test_0()
{
    Bus_t a = {5, 1};
    Bus_t b = {7, 0};

    Bus_t c = collide(a, b);
    assert(c.id == 5*7);
    assert(c.offs == 21);
}

void test_1()
{
    const char* sample = "939\n7,13,x,x,59,x,31,19";
    IntList_t ids = parse_ids(sample);

    assert(ids.len == 9);
    assert(ids.d[0] == 939);
    assert(ids.d[3] == 0);
    assert(ids.d[8] == 19);

    assert(part1_nextbus(ids) == 295);
    assert(part2_synctime(ids) == 1068781);

    ilist_free(&ids);
}

void test_2()
{
    const char* sample = "0\n1789,37,47,1889";
    IntList_t ids = parse_ids(sample);

    assert(part2_synctime(ids) == 1202161486);

    ilist_free(&ids);
}

char* read_text_file(const char* name)
{
    FILE *fp = fopen(name, "r");

    if (fp)
    {
        fseek(fp, 0L, SEEK_END);
        size_t sz = ftell(fp);
        rewind(fp);

        char* contents = (char*)malloc(sz+1);
        fread(contents, 1, sz, fp);

        fclose(fp);
        return contents;
    }
    else
    {
        fprintf(stderr, "Failed to open file %s\n", name);
        exit(EXIT_FAILURE);
    }
}

int main()
{
    printf("sizeof(size_t) = %d\n", sizeof(size_t));
    test_0();
    test_1();
    test_2();

    char* contents = read_text_file("../input.txt");
    IntList_t ids = parse_ids(contents);

    uint64_t p1 = part1_nextbus(ids);
    printf("Part 1 = %d\n", p1);

    // TODO: is_prime invariant check

    uint64_t p2 = part2_synctime(ids);
    printf("Part 2 = %llu\n", p2);

    ilist_free(&ids);
    free(contents);
    return 0;
}