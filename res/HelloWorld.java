public  class HelloWorld{

	int field1 = 10;
	double field2 = 4;
	String name = "HAHAHAHAHAHA";

	public int test(int p1){
		System.out.println(name + " " + (p1 + this.field1));
		return 4;
	}

	public static void main(String... args){
		System.out.println("Hello World!");
	}
}
