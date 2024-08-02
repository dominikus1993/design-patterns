
export type Operation = (a: number, b: number) => number;

abstract class TestMath {
    abstract createOperation(): Operation;

    doOperation(a: number, b: number): number {
        const op = this.createOperation();
        return op(a, b);
    }
}

export class TestAdd extends TestMath {
    createOperation(): Operation {
        return (a, b) => a + b;
    }
}

export class TestSub extends TestMath {
    createOperation(): Operation {
        return (a, b) => a - b;
    }
}