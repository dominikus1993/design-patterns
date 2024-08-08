export class InMemoryProvider {
    private nums = [0, 1]

    provide(): number[] {
        return this.nums;
    }
}

export class ConstructorProvider {

    constructor(private num: number) {

    }

    randomProvide(): number {
        return this.num;
    }
}

export class  InMemoryProviderAdapter extends InMemoryProvider {
    constructor(private p: ConstructorProvider) {
        super();
    }

    provide(): number[] {
        return [this.p.randomProvide()];
    }
}


export class Client {
    constructor(private provider: InMemoryProvider) {

    }

    sum() {
        let res = 0;
        for (const n of this.provider.provide()) {
            res += n;
        }
        return res;
    }
}