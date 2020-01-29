// We add volatile to prevent optimizations
int main () {
	volatile char a = 0x42;
	volatile char b = 0x13;
	return a + b;
}
