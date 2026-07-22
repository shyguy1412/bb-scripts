export type Result<T, E> = Ok<T, E> | Err<E, T>;
export const Result = class<T, E> {
    static Ok<T, E>(value: T): Result<T, E> {
        return new Ok(value);
    }
    static Err<T, E>(err: E): Result<T, E> {
        return new Err(err);
    }

    //in order to properly discriminate both Err and Ok need to keep track
    //of both generics, even if one of them is irrelevant to the type
    isError(): this is Err<E, T> {
        return this instanceof Err;
    }
    isOk(): this is Ok<T, E> {
        return this instanceof Ok;
    }

    map<M>(cb: (cur: T) => M): Result<M, E> {
        if (this.isError()) {
            return this as unknown as Result<never, E>;
        }

        if (this.isOk()) {
            return Result.Ok(cb(this.value));
        }

        throw new Error(
            'Result: Invariance Broken, is neither Err nor Ok variant',
        );
    }

    mapErr<M>(cb: (cur: E) => M): Result<T, M> {
        if (this.isOk()) {
            return this as unknown as Result<T, never>;
        }

        if (this.isError()) {
            return Result.Err(cb(this.error));
        }

        throw new Error(
            'Result: Invariance Broken, is neither Err nor Ok variant',
        );
    }

    andThen<M>(cb: (cur: T) => Result<M, E>): Result<M, E> {
        if (this.isOk()) {
            return cb(this.value);
        }

        if (this.isError()) {
            return this as unknown as Result<never, E>;
        }

        throw new Error(
            'Result: Invariance Broken, is neither Err nor Ok variant',
        );
    }

    unwrap(): T {
        if (this.isError()) {
            throw this.error;
        }

        if (this.isOk()) {
            return this.value;
        }

        throw new Error(
            'Result: Invariance Broken, is neither Err nor Ok variant',
        );
    }

    unwrapOr<O>(or: O): T | O {
        if (this.isError()) {
            return or;
        }

        if (this.isOk()) {
            return this.value;
        }

        throw new Error(
            'Result: Invariance Broken, is neither Err nor Ok variant',
        );
    }

    static async fromPromise<T>(promise: Promise<T>): Promise<Result<T, any>> {
        return await promise.then(Result.Ok).catch(Result.Err) as Result<
            T,
            any
        >;
    }
};

class Ok<T, N> extends Result<T, N> {
    #inner: T;
    constructor(value: T) {
        super();
        this.#inner = value;
    }

    get value() {
        return this.#inner;
    }
}

class Err<E, N> extends Result<N, E> {
    #inner: E;
    constructor(err: E) {
        super();
        this.#inner = err;
    }

    get error() {
        return this.#inner;
    }
}
