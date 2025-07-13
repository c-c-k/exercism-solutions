"""Functions to manage and organize queues at Chaitana's roller coaster."""


def add_me_to_the_queue(
    express_queue: list,
    normal_queue: list,
    ticket_type: int,
    person_name: str,
) -> list:
    """Add a person to the 'express' or 'normal' queue depending on the ticket number.

    :param express_queue: names in the Fast-track queue.
    :param normal_queue: names in the normal queue.
    :param ticket_type: type of ticket. 1 = express, 0 = normal.
    :param person_name: name of person to add to a queue.
    :return: the (updated) queue the name was added to.
    """

    if ticket_type == 1:
        queue = express_queue
    else:
        queue = normal_queue

    queue.append(person_name)
    return queue


def find_my_friend(queue: list, friend_name: str) -> int:
    """Search the queue for a name and return their queue position (index).

    :param queue: names in the queue.
    :param friend_name: name of friend to find.
    :return: index at which the friends name was found.
    """

    return queue.index(friend_name)


def add_me_with_my_friends(queue: list, index: int, person_name: str) -> list:
    """Insert the late arrival's name at a specific index of the queue.

    :param queue: names in the queue.
    :param index: the index at which to add the new name.
    :param person_name: the name to add.
    :return: queue updated with new name.
    """

    queue.insert(index, person_name)
    return queue


def remove_the_mean_person(queue: list, person_name: str) -> list:
    """Remove the mean person from the queue by the provided name.

    :param queue: names in the queue.
    :param person_name: name of mean person.
    :return: queue update with the mean persons name removed.
    """

    queue.remove(person_name)
    return queue


def how_many_namefellows(queue: list, person_name: str) -> int:
    """Count how many times the provided name appears in the queue.

    :param queue: names in the queue.
    :param person_name: name you wish to count or track.
    :return: the number of times the name appears in the queue.
    """

    return queue.count(person_name)


def remove_the_last_person(queue: list) -> str:
    """Remove the person in the last index from the queue and return their name.

    :param queue: names in the queue.
    :return: name that has been removed from the end of the queue.
    """

    return queue.pop()


def sorted_names(queue: list) -> list:
    """Sort the names in the queue in alphabetical order and return the result.

    :param queue: names in the queue.
    :return: copy of the queue in alphabetical order.
    """

    return sorted(queue)
