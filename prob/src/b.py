def solve():
    import sys

    input_data = sys.stdin.read().split()

    test_cases = int(input_data[0])
    data_index = 1
    result_lines = []
    for _ in range(test_cases):
        command_count = int(input_data[data_index])
        data_index += 1
        start_position = int(input_data[data_index])
        data_index += 1
        max_time = int(input_data[data_index])
        data_index += 1
        command_sequence = input_data[data_index]
        data_index += 1
        prefix_sums = [0] * (command_count + 1)
        for i in range(1, command_count + 1):
            if command_sequence[i - 1] == "L":
                prefix_sums[i] = prefix_sums[i - 1] - 1
            else:
                prefix_sums[i] = prefix_sums[i - 1] + 1
        first_reset_time = -1
        for i in range(1, command_count + 1):
            if start_position + prefix_sums[i] == 0:
                first_reset_time = i
                break
        if first_reset_time == -1 or first_reset_time > max_time:
            result_lines.append("0")
            continue
        reset_count = 1
        time_used = first_reset_time
        cycle_time = -1
        for j in range(1, command_count + 1):
            if prefix_sums[j] == 0:
                cycle_time = j
                break
        if cycle_time == -1:
            result_lines.append(str(reset_count))
            continue
        additional_cycles = (max_time - time_used) // cycle_time
        reset_count += additional_cycles
        result_lines.append(str(reset_count))
    sys.stdout.write("\n".join(result_lines))


if __name__ == "__main__":
    solve()
