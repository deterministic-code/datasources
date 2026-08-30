export interface IMapper<T> {
  to(value: T): T;
  from(value: T): T;
}
