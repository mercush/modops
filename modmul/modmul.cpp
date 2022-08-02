#include "cstdint"
#include <iostream>
#include <bitset>
#include "stdlib.h"
using namespace std;

uint32_t P = 15 * (uint32_t(1) << 27) + 1;
uint32_t M = 0x88000001;
uint32_t OptP = 2013265921;
uint32_t OptM = 2290649223;

uint32_t Risc0Mul(uint32_t a, uint32_t b) {
  // This uses some algorithm that I haven't seen before
  uint64_t o64 = uint64_t(a) * uint64_t(b);
  uint32_t low = -uint32_t(o64);
  uint32_t red = M * low;
  o64 += uint64_t(red) * uint64_t(P);
  uint32_t ret = o64 >> 32;
  return (ret >= P ? ret - P : ret);
}
// uint32_t OptP = 4294967291;
// uint64_t OptM = 4294967301;
// uint32_t n = 32;
uint32_t n = 31;
uint32_t OptMul(uint32_t a, uint32_t b) {
  uint64_t o64 = uint64_t(a) * uint64_t(b);
  uint64_t l1 = ((o64 >> n) * OptM) >> n;
  o64 -= l1 * OptP;
  uint32_t i = 0;
  while (o64 >= OptP && i < 4) {
    o64 -= OptP;
    i++;
  }
  return uint32_t(o64);
}

uint32_t DirMul(uint32_t a, uint32_t b) {
  return (uint64_t(a) * uint64_t(b)) % OptP;
}

int main() {
  uint32_t ra = rand() % P;
  uint32_t rb = rand() % P;
  freopen("out.csv","w",stdout);
  cout << "direct, "  << "domb, " << "r0" << "\n";
  for (int i = 0; i < 10000; i++) {
    clock_t dirBegin = clock();
    DirMul(ra, rb);
    float dirTime = float(clock() - dirBegin);
    clock_t optBegin = clock();
    OptMul(ra, rb);
    float optTime = float(clock() - optBegin);
    clock_t r0Begin = clock();
    Risc0Mul(ra, rb);
    float r0Time = float(clock() - r0Begin);
    cout << dirTime << ", " << optTime << ", " <<r0Time << "\n";
  }
  return 0;
}