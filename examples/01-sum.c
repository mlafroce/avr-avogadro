// We add volatile to prevent optimizations
int main () {
	volatile char a = 0x42;
	volatile char b = 0xB3;
	return a + b;
}
