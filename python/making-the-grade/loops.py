"""Functions for organizing and calculating student exam scores."""

FAIL_THRESHOLD = 40


def round_scores(student_scores: list[int | float]) -> list[int]:
    """Round all provided student scores.

    :param student_scores:float or int of student exam scores.
    :return: student scores *rounded* to nearest integer value.
    """

    rounded_scores = []
    for score in student_scores:
        rounded_scores.append(round(score))
    return rounded_scores


def count_failed_students(student_scores: list[int]) -> int:
    """Count the number of failing students out of the group provided.

    :param student_scores: list of student scores.
    :return: count of student scores at or below `FAIL_THRESHOLD`.
    """

    count = 0
    for score in student_scores:
        if score <= FAIL_THRESHOLD:
            count += 1
    return count


def above_threshold(student_scores: list[int], threshold: int) -> list[int]:
    """Determine how many of the provided student scores were 'the best' based on the provided threshold.

    :param student_scores: list of scores.
    :param threshold: threshold to cross to be in the "best" scores.
    :return: list of scores that are at or above the "best" threshold.
    """

    best_scores: list[int] = []
    for score in student_scores:
        if score >= threshold:
            best_scores.append(score)
    return best_scores


def letter_grades(highest: int) -> list[int]:
    """Create a list of grade thresholds based on the provided highest grade.

    :param highest: highest exam score.
    :return: list of lower threshold scores for each D-A letter grade interval.
            For example, where the highest score is 100,
            and failing is <= `FAIL_THRESHOLD`,
            The result would be [41, 56, 71, 86]:

            41 <= "D" <= 55
            56 <= "C" <= 70
            71 <= "B" <= 85
            86 <= "A" <= 100
    """

    interval = int((highest - FAIL_THRESHOLD) / 4)
    thresholds: list[int] = []
    for threshold in range(FAIL_THRESHOLD + 1, highest, interval):
        thresholds.append(threshold)
    return thresholds


def student_ranking(
    student_scores: list[int], student_names: list[str]
) -> list[str]:
    """Organize the student's rank, name, and grade information in descending order.

    :param student_scores: list of scores in descending order.
    :param student_names: list of names by exam score in descending order.
    :return: list of strings in format ["<rank>. <student name>: <score>"].
    """

    ranked_scores: list[str] = []
    for index, name in enumerate(student_names):
        ranked_scores.append(f"{index + 1}. {name}: {student_scores[index]}")
    return ranked_scores


def perfect_score(students_info: list[list[str | int]]) -> list[str | int]:
    """Create a list that contains the name and grade of the first student to make a perfect score on the exam.

    :param student_info: list of [<student name>, <score>] lists.
    :return: first `[<student name>, 100]` or `[]` if no student score of 100 is found.
    """

    top_scorer_info = []
    for student_info in students_info:
        if student_info[1] == 100:
            top_scorer_info = student_info
            break

    return top_scorer_info
