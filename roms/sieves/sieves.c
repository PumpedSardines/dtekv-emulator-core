char *sieves = (char *)0x10000;

void prime_sieves() {
  int i, j;
  for (i = 2; i < 100; i++) {
    sieves[i] = 1;
  }
  for (i = 2; i < 100; i++) {
    if (sieves[i]) {
      for (j = i + i; j < 100; j += i) {
        sieves[j] = 0;
      }
    }
  }
}
