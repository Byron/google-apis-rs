from copy import deepcopy
import sys

# COPY FROM `mako-render` TO CONVERT DATA FOR TESTS

# From https://github.com/Byron/bcore
class DictObject(object):

    """An object which wraps a dictionary to allow object.key access.
    If the source dictionary doesn't contain any sub-dictionaries, the input
    dict will be referenced. Otherwise it will be copied.

    An attribute error is raised if a value is not accessible.

    Please note that you cannot access dict keys which are not valid attribute names.
    """

    _default_dict = dict()
    _unpackable_types = (dict, tuple, list)

    def __init__(self, indict=_default_dict):
        """Initialize this instance from an input dictionary. If it contains other dictionaries, those will
        trigger their parent dictionaries to be copied, as they will be used as DictObject themselves and
        placed in the copy accordingly.
        NOTE: other DictObjects are used by reference. Generally, this type tries to perform the least
        amount of copying possible."""
        if indict is self._default_dict:
            return
        # end handle default instantiation, which makes us empty
        if isinstance(indict, DictObject):
            self.__dict__ = indict.__dict__
            return
        # END handle special case, be a reference
        dct = indict
        for key, val in dct.items():
            if isinstance(val, self._unpackable_types):
                dct = None
                break
        # END for each key-value pair

        if dct is None:
            dct = dict(indict)

            def unpack(val):
                """unpack helper"""
                if isinstance(val, dict):
                    val = DictObject(val)
                elif isinstance(val, (tuple, list)):
                    val = type(val)(unpack(item) for item in val)
                return val
            # END unpack
            for key, val in dct.items():
                dct[key] = unpack(val)
            # END for each k,v pair
        # END handle recursive copy
        self.__dict__ = dct

    def __str__(self):
        return str(self.__dict__)

    def __repr__(self):
        return repr(self.__dict__)

    def __getitem__(self, name):
        try:
            return getattr(self, name)
        except AttributeError:
            raise KeyError(name)
        # end convert exception

    def __setitem__(self, name, value):
        setattr(self, name, value)

    def __contains__(self, name):
        return name in self.__dict__

    def __len__(self):
        return len(self.__dict__)

    def __iter__(self):
        return iter(self.__dict__)

    def __eq__(self, other):
        """Compares a possibly expensive comparison"""
        if isinstance(other, DictObject):
            # EXPENSIVE !
            return self.to_dict() == other.to_dict()
        elif isinstance(other, dict):
            return self.to_dict() == other
        # end handle type of other
        return self is other

    def update(self, other, **kwargs):
        """Similar to dict.update"""
        items = other
        if hasattr(other, 'keys'):
            items = other.items()
        for item_list in (items, kwargs.items()):
            for k, v in item_list:
                setattr(self, k, v)
        # end for each item list

    def to_dict(self, recursive=False):
        """@return ourselves as normal dict
        @param recursive if True, a recursive copy will be returned if required."""
        if recursive:
            def obtain_needs_copy(value):
                """figure out if a copy is required"""
                if isinstance(value, DictObject):
                    return True
                if isinstance(value, (tuple, list, set)):
                    for item in value:
                        if obtain_needs_copy(item):
                            return True
                        # end check needs copy
                    # end for each item in value
                # end if instance is iterable
                return False
            # end check needs copy

            def unpack(val):
                """unpack val recursively and copy it gently"""
                if isinstance(val, DictObject):
                    val = val.to_dict(recursive)
                elif isinstance(val, (tuple, list, set)):
                    val = type(val)(unpack(item) for item in val)
                # end handle type resolution
                return val
            # end unpack

            needs_copy = False
            for value in self.__dict__.values():
                if obtain_needs_copy(value):
                    needs_copy = True
                    break
                # end check value
            # END for each value

            if needs_copy:
                new_dict = dict()
                for key, val in self.__dict__.items():
                    new_dict[key] = unpack(val)
                # END for each key, value pair
                return new_dict
            # else:
            #   just fall through and return ourselves as dictionary

        # END handle recursion
        return self.__dict__

    def copy(self):
        """@return a (deep) copy of self"""
        return type(self)(self.to_dict())

    def clone(self):
        """@return a deep copy of this dict. This onyl means that the key-sets are independent. However, the
        values are still shared, which matters in case of lists for instance"""
        return type(self)(deepcopy(self.to_dict(recursive=True)))

    def inversed_dict(self):
        """@return new dictionary which uses this dicts keys as values, and values as keys
        @note duplicate values will result in just a single key, effectively drupping items.
        Use this only if you have unique key-value pairs"""
        return dict(list(zip(list(self.__dict__.values()), list(self.__dict__.keys()))))

    def get(self, name, default=None):
        """as dict.get"""
        return self.__dict__.get(name, default)

    def keys(self):
        """as dict.keys"""
        return list(self.__dict__.keys())

    def values(self):
        """as dict.values"""
        return list(self.__dict__.values())

    def items(self):
        """as dict.items"""
        return list(self.__dict__.items())

    def _items(self):
        """as dict.items, avoiding name clashes"""
        return list(self.__dict__.items())

    def pop(self, key, default=sys):
        """as dict.pop"""
        if default is sys:
            return self.__dict__.pop(key)
        else:
            return self.__dict__.pop(key, default)
        # end assure semantics are kept
