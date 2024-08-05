export interface Builder<T> {
    build(): T
    reset(): void
}

export interface Person {
    readonly name: string
    readonly age: number
}

export class PersonBuilder implements Builder<Person> {
    private name: string;
    private age: number;

    withName(name:string): this {
        this.name = name;
        return this;
    }

    withAge(age: number): this {
        this.age = age;
        return this;
    }

    build(): Person {
        return {
            name: this.name,
            age: this.age
        }
    }

    reset(): void {
        throw new Error("Method not implemented.");
    }
}