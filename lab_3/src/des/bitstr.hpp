#pragma once

#include <cstdint>
#include <type_traits>
#include <stdexcept>
#include <cstring>
#include <iostream>

template<unsigned int N>
class bitstr
{
private:
    bool data[N];

public:
    bitstr(uint64_t value = 0)
    {
        for (int i = N - 1; i >= 0; i--)
        {
            data[i] = (value & 1);
            value >>= 1;
        }
    }

    bitstr& operator=(const bitstr& bs)
    {
        for (int i = 0; i < N; i++)
            data[i] = bs[i];
        return *this;
    }

    const bool& operator[](size_t index) const
    {
        if (index >= N)
            throw std::out_of_range("index out of range");

        return data[index];
    }

    bool& operator[](size_t index)
    {
        if (index >= N)
            throw std::out_of_range("index out of range");

        return data[index];
    }

    template<unsigned int _Count>
    bitstr<_Count> substr(unsigned int from = 0) const
    {
        bitstr<_Count> res;

        for (int i = 0; i < _Count; i++)
            res[i] = data[from + i];

        return res;
    }

    template<unsigned int M>
    void set(int pos, const bitstr<M> bs)
    {
        for (int i = 0; i < M; i++)
            data[i + pos] = bs[i];
    }

    bitstr<N> operator xor(const bitstr<N>& bs)
    {
        bitstr<N> res;

        for (int i = 0; i < N; i++)
            res[i] = data[i] xor bs[i];

        return res;
    }

    void rotate_left(int pos)
    {
        bool old_data[N];
        memcpy(old_data, data, sizeof(data));

        for (int i = 0; i < N; i++)
            data[i] = old_data[(i + pos + N) % N];
    }

    operator uint64_t()
    {
        uint64_t res = 0;

        for (int i = 0; i < N; i++)
            res = (res << 1) | (uint64_t) data[i];

        return res;
    }

    template<typename T>
    void permute(const T p[], int offset = 0)
    {
        bool old_data[N];
        memcpy(old_data, data, sizeof(data));

        for (int i = 0; i < N; i++)
            data[i] = old_data[p[i] - offset];
    }

    template<unsigned int M, typename T>
    bitstr<M> extend(const T s[M], int offset = 0) const
    {
        bitstr<M> res;

        for (int i = 0; i < M; i++)
            res[i] = data[s[i] - offset];

        return res;
    }

    template<unsigned int M>
    bitstr<N + M> chain(const bitstr<M> bs) const
    {
        bitstr<N + M> res;

        for (int i = 0; i < N; i++)
            res[i] = data[i];
        
        for (int i = 0; i < M; i++)
            res[N + i] = bs[i];

        return res;
    }
};

template<unsigned int N>
std::ostream& operator<<(std::ostream& stream, const bitstr<N> bs)
{
    for (int i = 0; i < N; i++)
    {
        if (i != 0 && i % 4 == 0)
            stream << ' ';
        stream << (bs[i] ? '1' : '0');
    }

    return stream;
}

