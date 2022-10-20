//
// Created by moosavi on 10/20/22.
//


#include <gtest/gtest.h>
#include <gmock/gmock.h>

#include "../include/Time.h"

using testing::Gt;
using testing::Eq;
using testing::Lt;

TEST(TimeTest, FailTime) {
    EXPECT_ANY_THROW(Time("00:00:12"));
    EXPECT_ANY_THROW(Time("00:2:000"));
    EXPECT_ANY_THROW(Time("0100:000"));
    EXPECT_ANY_THROW(Time("0:0:500"));
    EXPECT_ANY_THROW(Time("00-10-000"));
    EXPECT_ANY_THROW(Time("00-10:000"));
    EXPECT_ANY_THROW(Time("00:10-000"));
}

TEST(TimeTest, CorrectTimme) {
    EXPECT_NO_THROW(Time("00:00:120"));
    EXPECT_NO_THROW(Time("00:05:100"));
}

TEST(TimeTest, Compare) {
    auto time1 = Time("11:00:000");
    auto time2 = Time("10:00:000");
    auto time3 = Time("00:00:100");
    auto time4 = Time("00:00:101");

    EXPECT_THAT(time1, Gt(time2));
    EXPECT_THAT(time2, Eq(time2));
    EXPECT_THAT(time2, Gt(time3));
    EXPECT_THAT(time3, Lt(time4));
    EXPECT_THAT(time1, Lt("11:10:000"));
    EXPECT_THAT(time1, Gt("09:10:000"));
    EXPECT_THAT(time1, Eq("11:00:000"));
}

TEST(TimeTest, OperatorStar) {
    auto time1 = Time("00:01:000");
    auto time2 = Time("00:02:000");
    EXPECT_THAT(time1 * 2, Eq(time2));
}

TEST(TimeTest, Round) {
    auto time1 = Time("00:01:000");
    auto time2 = Time("00:01:000", 1);
    auto time3 = Time("00:01:000", 2);

    EXPECT_THAT(time1, Lt(time3));
    EXPECT_THAT(time1, Eq(time2));
}